import { LiquidityBootstrappingPool } from "@/defi/types";
import { DEFAULT_NETWORK_ID, fetchSpotPrice, uniswapCalculator } from "@/defi/utils";
import { fetchAuctions, fetchTrades } from "@/defi/utils/pablo/auctions";
import { useAppSelector } from "@/hooks/store";
import { MockedAsset } from "@/store/assets/assets.types";
import { useAssetBalance } from "@/store/assets/hooks";
import useStore from "@/store/useStore";
import BigNumber from "bignumber.js"
import { useSnackbar } from "notistack";
import { useCallback, useMemo, useState } from "react";
import { useDotSamaContext, useParachainApi, usePendingExtrinsic, useSelectedAccount } from "substrate-react";
import { useAsset } from "../assets/useAsset";

export const useBuyForm = (): {
  balanceBase: BigNumber;
  balanceQuote: BigNumber;
  isValidBaseInput: boolean;
  setIsValidBaseInput: (validity: boolean) => void;
  isValidQuoteInput: boolean;
  setIsValidQuoteInput: (validity: boolean) => void;
  quoteAsset: MockedAsset | undefined,
  baseAsset: MockedAsset | undefined,
  minimumReceived: BigNumber;
  baseAmount: BigNumber;
  quoteAmount: BigNumber;
  feeCharged: BigNumber;
  slippageAmount: BigNumber;
  selectedAuction: LiquidityBootstrappingPool;
  isBuyButtonDisabled: boolean;
  refreshAuctionData: () => void;
  isPendingBuy: boolean;
  onChangeTokenAmount: (changedSide: "quote" | "base", amount: BigNumber) => Promise<{
    minReceive: BigNumber;
    tokenOutAmount: BigNumber;
    feeCharged: BigNumber;
    slippageAmount: BigNumber;
  }>
} => {
  const { enqueueSnackbar } = useSnackbar();
  const { extensionStatus } = useDotSamaContext();
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const selectedAccount = useSelectedAccount(DEFAULT_NETWORK_ID);
  const { auctions: { activeLBP }, putStatsActiveLBP, putHistoryActiveLBP } = useStore();
  const slippage = useAppSelector(
    (state) => state.settings.transactionSettings.tolerance
  );

  const baseAsset = useAsset(activeLBP.pair.base.toString());
  const quoteAsset = useAsset(activeLBP.pair.quote.toString());

  const balanceBase = useAssetBalance(DEFAULT_NETWORK_ID, baseAsset ? baseAsset.network[DEFAULT_NETWORK_ID] : "none")
  const balanceQuote = useAssetBalance(DEFAULT_NETWORK_ID, quoteAsset ? quoteAsset.network[DEFAULT_NETWORK_ID] : "none")

  const [isValidBaseInput, setIsValidBaseInput] = useState(false);
  const [isValidQuoteInput, setIsValidQuoteInput] = useState(false);

  const [minimumReceived, setMinimumReceived] = useState(new BigNumber(0));
  const [slippageAmount, setSlippageAmount] = useState(new BigNumber(0));
  const [feeCharged, setFeeCharged] = useState(new BigNumber(0));

  const [tokenAmounts, setTokenAmounts] = useState({
    baseAmount: new BigNumber(0),
    quoteAmount: new BigNumber(0),
  })

  const resetTokenAmounts = () => setTokenAmounts({
    baseAmount: new BigNumber(0),
    quoteAmount: new BigNumber(0)
  })

  const onChangeTokenAmount = async (
    changedSide: "base" | "quote",
    amount: BigNumber
  ) => {
    if (
      parachainApi &&
      activeLBP
    ) {
      const { base, quote } = activeLBP.pair;
      const { feeRate } = activeLBP.feeConfig;
      let feePercentage = new BigNumber(feeRate).toNumber();

      let pair = { base: base.toString(), quote: quote.toString() };

      const oneBaseInQuote = await fetchSpotPrice(
        parachainApi,
        pair,
        activeLBP.poolId
      );
      const { minReceive, tokenOutAmount, feeChargedAmount, slippageAmount } =
        uniswapCalculator(
          changedSide,
          false,
          amount,
          oneBaseInQuote,
          slippage,
          feePercentage
        );

      setTokenAmounts({
        quoteAmount: changedSide === "base" ? tokenOutAmount : amount,
        baseAmount: changedSide === "quote" ? tokenOutAmount : amount,
      });
      setMinimumReceived(minReceive);
      setFeeCharged(feeChargedAmount);
      setSlippageAmount(slippageAmount);
      return {
        minReceive,
        tokenOutAmount,
        feeCharged,
        slippageAmount,
      };
    } else {
      resetTokenAmounts();
      console.error(`Error LBP`);
      enqueueSnackbar(`Error LBP`);
      return {
        minReceive: new BigNumber(0),
        tokenOutAmount: new BigNumber(0),
        feeCharged: new BigNumber(0),
        slippageAmount: new BigNumber(0),
      };
    }
  };

  const refreshAuctionData = useCallback(async () => {
    const { poolId } = activeLBP;
    if (parachainApi && poolId !== -1) {
      const stats = await fetchAuctions(parachainApi, activeLBP);
      const trades = await fetchTrades(activeLBP);
      putStatsActiveLBP(stats);
      putHistoryActiveLBP(trades);
    }
  }, [activeLBP, putHistoryActiveLBP, putStatsActiveLBP, parachainApi])

  const { baseAmount, quoteAmount } = tokenAmounts;
  
  const isPendingBuy = usePendingExtrinsic(
    "exchange",
    "dexRouter",
    selectedAccount ? selectedAccount.address : ""
  );

  const isBuyButtonDisabled = useMemo(() => {
    return extensionStatus !== "connected" || !isValidBaseInput || !isValidQuoteInput || isPendingBuy
  }, [isValidBaseInput, isValidQuoteInput, extensionStatus, isPendingBuy])


  return {
    balanceBase,
    balanceQuote,
    isValidBaseInput,
    setIsValidBaseInput,
    isValidQuoteInput,
    setIsValidQuoteInput,
    quoteAsset,
    baseAsset,
    minimumReceived,
    baseAmount,
    quoteAmount,
    slippageAmount,
    feeCharged,
    isBuyButtonDisabled,
    selectedAuction: activeLBP,
    refreshAuctionData,
    onChangeTokenAmount,
    isPendingBuy
  }
}
import { Add } from "@mui/icons-material";
import {
  alpha,
  Box,
  Paper,
  Typography,
  useMediaQuery,
  useTheme,
} from "@mui/material";
import Image from "next/image";
import { useCallback } from "react";

type AccountIndicatorProps = {
  label: string;
  onClick: () => void;
  isEthereumConnected: boolean;
  isPolkadotConnected: boolean;
};

export const AccountIndicator: React.FC<AccountIndicatorProps> = ({
  label,
  onClick,
  isEthereumConnected = false,
  isPolkadotConnected,
}) => {
  const theme = useTheme();
  const isMobile = useMediaQuery(theme.breakpoints.down("sm"));
  const polkaIcon = "/networks/picasso.svg";
  const ethIcon = "/networks/mainnet.svg";

  const networkIcons = useCallback(() => {
    if (isEthereumConnected && isPolkadotConnected) {
      return (
        <>
          <Box sx={{ display: "flex" }}>
            <Image src={polkaIcon} width="24" height="24" alt="Account" />
          </Box>
          <Box sx={{ display: "flex", marginLeft: -1.5 }}>
            <Image src={ethIcon} width="24" height="24" alt="Account" />
          </Box>
        </>
      );
    } else if (isEthereumConnected || isPolkadotConnected) {
      const icon = isEthereumConnected ? ethIcon : polkaIcon;
      return (
        <Box sx={{ display: "flex" }}>
          <Image src={icon} width="24" height="24" alt="Account" />
        </Box>
      );
    } else {
      return (
        <>
          <Box sx={{ display: "flex" }}>
            <Image
              style={{ filter: "grayscale(100%)" }}
              src={polkaIcon}
              width="24"
              height="24"
              alt="Account"
            />
          </Box>
          <Box sx={{ display: "flex" }}>
            <Image
              style={{ filter: "grayscale(100%)" }}
              src={ethIcon}
              width="24"
              height="24"
              alt="Account"
            />
          </Box>
        </>
      );
    }
  }, [isEthereumConnected, isPolkadotConnected]);

  return (
    <Paper
      onClick={onClick}
      sx={{
        display: "flex",
        alignContent: "center",
        gap: theme.spacing(2),
        flexShrink: 0,
        background: alpha(
          theme.palette.primary.main,
          theme.custom.opacity.light
        ),
        cursor: "pointer",
        "&:hover": {
          background: alpha(
            theme.palette.primary.main,
            theme.custom.opacity.main
          ),
        },
      }}
    >
      <Box
        sx={{
          height: theme.spacing(3),
          display: "flex",
          flexGrow: isMobile ? 1 : undefined,
          justifyContent: isMobile ? "center" : undefined,
        }}
      >
        {networkIcons()}
      </Box>

      {!isMobile ? (
        <>
          <Typography variant="body2">{label}</Typography>
          {!isEthereumConnected && isPolkadotConnected ? <Add /> : null}
        </>
      ) : null}
    </Paper>
  );
};

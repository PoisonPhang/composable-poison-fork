/**
 * Creating a sidebar enables you to:
 - create an ordered group of docs
 - render a sidebar for each doc of that group
 - provide next/previous navigation

 The sidebars can be generated from the filesystem, or explicitly defined here.

 Create as many sidebars as you want.
 */

const isProd = process.env.NODE_ENV === 'production';

// @ts-check

/** @type {import('@docusaurus/plugin-content-docs').SidebarsConfig} */
const sidebars = {
	// By default, Docusaurus generates a sidebar from the docs folder structure
	// tutorialSidebar: [{type: 'autogenerated', dirName: '.'}],

	// But you can create a sidebar manually
	internalSidebar: [{ type: 'autogenerated', dirName: 'internal' }],
	tutorialSidebar: [
		'intro',
		{
			type: 'category',
			label: 'Parachains',

      link: {
        type: 'generated-index',
        slug: 'parachains',
      },
      collapsible: false,
      items: [
        {
          type: 'category',
          label: 'Picasso',
          link: {
            type: 'doc',
            id: 'parachains/picasso-parachain-overview'
          },
          collapsible: true,
          collapsed: true,
          items: [
            'parachains/picasso/crowdloan',
            'parachains/picasso/governance',
            {
            type: 'category',
            label: 'Picasso Tokenomics',
            link: {
              type: 'doc',
              id: 'parachains/picasso/tokenomics'
            },
            collapsible: true,
            collapsed: true,
            items:[
              'parachains/picasso/token-transparency',
              'parachains/picasso/pica-use-cases'
            ]
            },
            {
              type: 'category',
              label: 'xPICA fNFTs',
              link: {
                type: 'doc',
                id: 'products/xpica-fnft-overview'
              },
              collapsible: true,
              collapsed: true,
              items: [
                'products/xpica-fnft/picasso-treasury',
                'products/xpica-fnft/use-cases'
              ],
            },
          ],
        },
        {
          type: 'category',
          label: 'Composable',
          link: {
            type: 'doc',
            id: 'parachains/composable-parachain-overview'
          },
          collapsible: true,
          collapsed: true,
          items: [
            'parachains/composable/composable-crowdloan',
            'parachains/composable/DOT-purchase-log',
            'parachains/composable/LAYR-tokenomics',
          ],
        }
      ]
    },
    {
      type: 'category',
      label: 'Products',
      link: {
        type: 'generated-index',
        slug: 'products',
      },
      collapsible: false,
      items: [
        {
          type: 'category',
          label: 'XCVM',
          link: {
            type: 'doc',
            id: 'products/xcvm'
          },
          collapsible: true,
          collapsed: true,
          items: [
            'products/xcvm/how-it-works',
            'products/xcvm/routing-layer',
            'products/xcvm/routing-layer-libraries',
            'products/xcvm/writing-smart-contracts-with-cosmwasm',
            {
                type: 'category',
                label: 'Use Cases',
                link: {
                    type: 'generated-index',
                    slug: 'use-cases'
                },
                collapsible: false,
                items: [
                  'products/xcvm/use-cases/swap',
                ]
            }
          ],
        },
        {
          type: 'category',
          label: 'Centauri',
          link: {
            type: 'doc',
            id: 'products/centauri-overview'
          },
          collapsible: true,
          collapsed: true,
          items: [
            'products/centauri/light-clients',
            'products/centauri/merkle-mountain-ranges',
            'products/centauri/cosmos11-BEEFY-COSMOS-IBC-light-client',
            'products/centauri/expanding-ibc-protocol'
          ],
        },
        {
          type: 'category',
          label: 'Apollo',
          link: {
            type: 'doc',
            id: 'products/apollo-overview'
          },
          collapsible: true,
          collapsed: true,
          items: [
            'products/apollo/apollo-how-it-works',
            'products/apollo/apollo-deployment'
          ],
        },
        {
          type: 'category',
          label: 'Cubic',
          link: {
            type: 'doc',
            id: 'products/cubic-overview'
          },
          collapsible: true,
          collapsed: true,
          items: [
            'products/cubic/additional-details',
          ],
        },
        {
          type: 'category',
          label: 'Pablo',
          link: {
            type: 'doc',
            id: 'products/pablo-overview'
          },
          collapsible: true,
          collapsed: true,
          items: [
            'products/pablo/swaps-trading',
            'products/pablo/liquidity-provision',
            'products/pablo/auctions-bonding',
            'products/pablo/xPBLO-fNFT-staking',
            'products/pablo/cross-chain-DEX',
            'products/pablo/governance-tokenomics',
          ],
        },
        {
          type: 'category',
          label: 'Mosaic (Discontinued)',
          link: {
            type: 'doc',
            id: 'products/mosaic-overview'
          },
          collapsible: true,
          collapsed: true,
          items: [
            'products/mosaic/dynamic-fee-model',
            'products/mosaic/liquidity-forecasting',
            'products/mosaic/passive-liquidity-rebalancing',
            'products/mosaic/active-liquidity-management',
            'products/mosaic/single-sided-staking',
            'products/mosaic/additional-use-cases',
            'products/mosaic/mosaic-integrations',
            'products/mosaic/mosaic-pallet',
            'products/mosaic/mosaic-phase1-result',
            {
              type: 'category',
              label: 'Mural: NFT Transfers on Mosaic via the Summoner Vault',
              link: {
                type: 'doc',
                id: 'products/mosaic/mural-NFT-transfers/mural-NFT-transfers'
              },
              collapsible: true,
              collapsed: true,
              items: [
                'products/mosaic/mural-NFT-transfers/NFT-transfer-flow',
                'products/mosaic/mural-NFT-transfers/NFT-contract-details'
              ],
            },
          ],
        },
        {
          type: 'category',
          label: 'Parachain Vault Strategy (Discontinued)',
          link: {
            type: 'doc',
            id: 'products/parachain-vault-strategy'
          },
          collapsible: true,
          collapsed: true,
          items: [
            'products/parachain-vault-strategy/vault-process-in-detail/vault-process-in-detail',
            'products/parachain-vault-strategy/contracts-technical-details/contracts-technical-details',
          ],
        }
      ]
    },
    {
      type: 'category',
      label: 'User Guides',
      link: {
        type: 'generated-index',
        slug: 'user-guides',
      },
      collapsible: false,
      items: [
        {
          type: 'category',
          label: 'Accounts and Wallets',
          link: {
            type: 'generated-index',
            slug: 'accounts-wallets'
          },
          collapsible: true,
          items: [
            'user-guides/polkadotjs-extension-create-account',
            'user-guides/talisman-create-account',
            `user-guides/transfer-usdt-statemine-picasso`,
        ],
        },
        'user-guides/claim-rewards-guide',
        'user-guides/polkassembly-picasso-governance',
      ]
    },
    {
      type: 'category',
      label: 'Developer Guides',

			link: {
				type: 'generated-index',
				slug: 'developer-guides',
			},
			collapsible: false,
			items: [
				{
					type: 'category',
					label: 'Nix',
					link: {
						type: 'doc',
						id: 'nix',
					},
					collapsible: true,
					collapsed: true,
					items: [
						'nix/install',
						'nix/run-packages',
						'nix/development-environments',
						'nix/running-checks',
						'nix/defining-your-own-packages',
						'nix/composing-services-with-arion',
						'nix/editing-docs',
            'nix/troubleshooting',
					],
				},
				{
					type: 'category',
					label: 'Codespaces',
					link: {
						type: 'doc',
						id: 'codespaces',
					},
					collapsible: true,
					collapsed: true,
					items: [
						'codespaces/getting-started',
						{
							type: 'category',
							label: 'Using Codespaces',
							link: {
								type: 'doc',
								id: 'codespaces/using-codespaces',
							},
							collapsible: true,
							collapsed: false,
							items: ['codespaces/book', 'codespaces/substrate', 'codespaces/frontend', 'codespaces/runtime-tests'],
						},
					],
				},
				'developer-guides/oracle-set-up-guide',
				'developer-guides/collator-set-up-guide',
			],
		},
		{
			type: 'category',
			label: 'Ecosystem',

			link: {
				type: 'generated-index',
				slug: 'ecosystem',
			},
			collapsible: false,
			items: [
				'ecosystem/build-on-composable-ecosystem-development',
				'ecosystem/composable-grants',
				'ecosystem/business-line-development',
				'ecosystem/press-kit',
				'ecosystem/the-composable-team',
				'ecosystem/careers',
			],
		},
		{
			type: 'category',
			label: 'Audits And Fixes',

      link: {
        type: 'generated-index',
        slug: 'audits',
      },
      collapsible: false,
      items: [
        'audits/audit-results-recommendations-and-remediations',
      ]
    },
    {
      type: 'doc',
      label: 'FAQs',
      id: 'faqs/faqs',

    },
    {
      type: 'category',
      label: 'Legal Disclaimers and Disclosures',
      collapsible: true,
      collapsed: true,
      items:[
        'faqs/disclaimer-disclosure-pica-token',
        'faqs/risk-factors',
        'faqs/legal-disclaimer-disclosures',
        'faqs/terms-of-use',
      ]
    }
  ],
};

if (!isProd) {
  sidebars.tutorialSidebar.unshift({
    type: 'category',
    label: 'test-SCDI',
    link: {
      type: 'doc',
      id: 'testSCDI/entry',
    },
    collapsible: true,
    collapsed: true,
    items: [
      {
        type: 'link',
        label: 'test-SCDI',
        href: '/test-vm',
      },
    ],
  });
}

module.exports = sidebars;

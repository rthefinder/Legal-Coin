export type TokenLaunch = {
  mint: string;
  creator: string;
  supply: string;
  decimals: number;
  launchedAt: number;
};

export const SAFE_CHECKS = [
  'mint_authority_revoked',
  'freeze_authority_revoked',
  'single_tx_mint',
];

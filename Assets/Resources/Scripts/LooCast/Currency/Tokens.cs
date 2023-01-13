using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using Steamworks;

namespace LooCast.Currency
{
    using LooCast.Variable;
    using LooCast.Data;
    using LooCast.Util;

    [CreateAssetMenu(fileName = "Tokens", menuName = "Data/Currency/Tokens", order = 0)]
    public class Tokens : DynamicData
    {
        #region Classes
        [Serializable]
        private class DataContainer
        {
            [SerializeField] private int balance;
            [SerializeField] private int proposedBalanceChange;

            public DataContainer(IntVariable balance, IntVariable proposedBalanceChange)
            {
                this.balance = balance.Value;
                this.proposedBalanceChange = proposedBalanceChange.Value;
            }

            public DataContainer(int balance, int proposedBalanceChange)
            {
                this.balance = balance;
                this.proposedBalanceChange = proposedBalanceChange;
            }

            public IntVariable GetBalance()
            {
                return new IntVariable(balance);
            }

            public IntVariable GetProposedBalanceChange()
            {
                return new IntVariable(proposedBalanceChange);
            }
        }
        #endregion

        #region Fields
        public IntVariable Balance;
        public IntVariable ProposedBalanceChange;
        #endregion

        #region Unity Callbacks
        private void OnEnable()
        {
            
        }
        #endregion

        #region Methods
        public override void Save()
        {
            SerializationUtil.SaveData(new DataContainer(Balance, ProposedBalanceChange), $"Currency/Tokens.dat");
        }

        public override void Load()
        {
            DataContainer dataContainer = SerializationUtil.LoadData<DataContainer>("Currency/Tokens.dat");
            Balance = dataContainer.GetBalance();
            ProposedBalanceChange = dataContainer.GetProposedBalanceChange();

            Balance.OnValueChanged.AddListener(() =>
            {
                if (SteamManager.Initialized)
                {
                    SteamUserStats.GetStat("highscore_tokens_balance", out int highscore_tokens_balance);
                    if (Balance.Value > highscore_tokens_balance)
                    {
                        SteamUserStats.SetStat("highscore_tokens_balance", Balance.Value);
                    }
                    if (Balance.Value >= 420)
                    {
                        SteamUserStats.GetAchievement("The_Funny_Number_Harder", out bool achievementCompleted);
                        if (!achievementCompleted)
                        {
                            SteamUserStats.SetAchievement("The_Funny_Number_Harder");
                        }
                    }
                    SteamUserStats.StoreStats();
                }
            });
        }

        public override void LoadDefault()
        {
            DataContainer dataContainer = new DataContainer(0, 0);
            Balance = dataContainer.GetBalance();
            ProposedBalanceChange = dataContainer.GetProposedBalanceChange();

            Balance.OnValueChanged.AddListener(() =>
            {
                if (SteamManager.Initialized)
                {
                    SteamUserStats.GetStat("highscore_tokens_balance", out int highscore_tokens_balance);
                    if (Balance.Value > highscore_tokens_balance)
                    {
                        SteamUserStats.SetStat("highscore_tokens_balance", Balance.Value);
                    }
                    if (Balance.Value >= 420)
                    {
                        SteamUserStats.GetAchievement("The_Funny_Number_Harder", out bool achievementCompleted);
                        if (!achievementCompleted)
                        {
                            SteamUserStats.SetAchievement("The_Funny_Number_Harder");
                        }
                    }
                    SteamUserStats.StoreStats();
                }
            });
        }
        #endregion
    }
}

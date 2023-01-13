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

    [CreateAssetMenu(fileName = "Coins", menuName = "Data/Currency/Coins", order = 0)]
    public class Coins : DynamicData
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
            SerializationUtil.SaveData(new DataContainer(Balance, ProposedBalanceChange), $"Currency/Coins.dat");
        }

        public override void Load()
        {
            DataContainer dataContainer = SerializationUtil.LoadData<DataContainer>("Currency/Coins.dat");
            Balance = dataContainer.GetBalance();
            ProposedBalanceChange = dataContainer.GetProposedBalanceChange();

            Balance.OnValueChanged.AddListener(() =>
            {
                if (SteamManager.Initialized)
                {
                    SteamUserStats.GetStat("highscore_coins_balance", out int highscore_coins_balance);
                    if (Balance.Value > highscore_coins_balance)
                    {
                        SteamUserStats.SetStat("highscore_coins_balance", Balance.Value);
                    }
                    if (Balance.Value >= 42069)
                    {
                        SteamUserStats.GetAchievement("The_Most_Funny_Number", out bool achievementCompleted);
                        if (!achievementCompleted)
                        {
                            SteamUserStats.SetAchievement("The_Most_Funny_Number");
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
                    SteamUserStats.GetStat("highscore_coins_balance", out int highscore_coins_balance);
                    if (Balance.Value > highscore_coins_balance)
                    {
                        SteamUserStats.SetStat("highscore_coins_balance", Balance.Value);
                    }
                    if (Balance.Value >= 42069)
                    {
                        SteamUserStats.GetAchievement("The_Most_Funny_Number", out bool achievementCompleted);
                        if (!achievementCompleted)
                        {
                            SteamUserStats.SetAchievement("The_Most_Funny_Number");
                        }
                    }
                    SteamUserStats.StoreStats();
                }
            });
        }
        #endregion
    }
}

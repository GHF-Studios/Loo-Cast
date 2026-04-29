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

    [CreateAssetMenu(fileName = "Credits", menuName = "Data/Currency/Credits", order = 0)]
    public class Credits : DynamicGameData
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

        #region Methods
        public override void Save()
        {
            SerializationUtil.SaveData(new DataContainer(Balance, ProposedBalanceChange), $"Currency/Credits.dat");
        }

        public override void Load()
        {
            DataContainer dataContainer = SerializationUtil.LoadData<DataContainer>("Currency/Credits.dat");
            Balance = dataContainer.GetBalance();
            ProposedBalanceChange = dataContainer.GetProposedBalanceChange();

            Balance.OnValueChanged.AddListener(() =>
            {
                if (SteamManager.Initialized)
                {
                    SteamUserStats.GetStat("highscore_credits_balance", out int highscore_credits_balance);
                    if (Balance.Value > highscore_credits_balance)
                    {
                        SteamUserStats.SetStat("highscore_credits_balance", Balance.Value);
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
                    SteamUserStats.GetStat("highscore_credits_balance", out int highscore_credits_balance);
                    if (Balance.Value > highscore_credits_balance)
                    {
                        SteamUserStats.SetStat("highscore_credits_balance", Balance.Value);
                    }
                    SteamUserStats.StoreStats();
                }
            });
        }
        #endregion
    }
}

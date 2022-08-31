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
    [Serializable]
    public class Credits : ScriptableObject
    {
        private class DataContainer
        {
            [SerializeField] private int balance;
            [SerializeField] private int proposedBalanceChange;

            public DataContainer(IntVariable balance, IntVariable proposedBalanceChange)
            {
                this.balance = balance.Value;
                this.proposedBalanceChange = proposedBalanceChange.Value;
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
        public IntVariable Balance;
        public IntVariable ProposedBalanceChange;

        private void OnValidate()
        {
            Save(true);
        }

        private void OnEnable()
        {
            Load();
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

        private void OnDisable()
        {
            Save();
        }

        public void Save(bool saveDefault = false)
        {
            JSONUtil.SaveData(new DataContainer(Balance, ProposedBalanceChange), $"{(saveDefault ? "Default/" : "")}Currency/Credits.json");
        }

        public void Load()
        {
            DataContainer dataContainer = JSONUtil.LoadData<DataContainer>("Currency/Credits.json");
            Balance = dataContainer.GetBalance();
            ProposedBalanceChange = dataContainer.GetProposedBalanceChange();
        }
    } 
}

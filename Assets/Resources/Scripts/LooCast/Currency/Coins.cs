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
    [Serializable]
    public class Coins : ScriptableObject
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

        private void OnEnable()
        {
            Load();
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

        private void OnDisable()
        {
            Save();
        }

        public void Save(bool saveDefault = false)
        {
            JSONUtil.SaveData(new DataContainer(Balance, ProposedBalanceChange), $"{(saveDefault ? "Default/" : "")}Currency/Coins.json");
        }

        public void Load()
        {
            DataContainer dataContainer = JSONUtil.LoadData<DataContainer>("Currency/Coins.json");
            Balance = dataContainer.GetBalance();
            ProposedBalanceChange = dataContainer.GetProposedBalanceChange();
        }
    } 
}

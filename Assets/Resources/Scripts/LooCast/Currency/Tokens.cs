using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Currency
{
    using LooCast.Variable;
    using LooCast.Data;
    using LooCast.Util;

    [CreateAssetMenu(fileName = "Tokens", menuName = "Data/Currency/Tokens", order = 0)]
    [Serializable]
    public class Tokens : ScriptableObject, ISaveable
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
        }

        private void OnDisable()
        {
            Save();
        }

        public void Save(bool saveDefault = false)
        {
            JSONUtil.SaveData(new DataContainer(Balance, ProposedBalanceChange), $"{(saveDefault ? "Default/" : "")}Currency/Tokens.json");
        }

        public void Load()
        {
            DataContainer dataContainer = JSONUtil.LoadData<DataContainer>("Currency/Tokens.json");
            Balance = dataContainer.GetBalance();
            ProposedBalanceChange = dataContainer.GetProposedBalanceChange();
        }
    } 
}

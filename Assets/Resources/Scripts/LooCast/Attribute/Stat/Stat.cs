using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Attribute.Stat
{
    using LooCast.Variable;
    using LooCast.Data;
    using LooCast.Util;

    [Serializable]
    public abstract class Stat : ScriptableObject, ISaveable
    {
        private class DataContainer
        {
            [SerializeField] private string attributeName;
            [SerializeField] private int level;
            [SerializeField] private int maxLevel;
            [SerializeField] private int proposedLevelChange;

            public DataContainer(Attribute attribute, IntVariable level, IntVariable maxLevel, IntVariable proposedLevelChange)
            {
                attributeName = attribute.AttributeName;
                this.level = level.Value;
                this.maxLevel = maxLevel.Value;
                this.proposedLevelChange = proposedLevelChange.Value;
            }

            public Attribute GetAttribute(Attributes allAttributes)
            {
                return allAttributes.GetAttribute(attributeName);
            }

            public IntVariable GetLevel()
            {
                return new IntVariable(level);
            }

            public IntVariable GetMaxLevel()
            {
                return new IntVariable(maxLevel);
            }

            public IntVariable GetProposedLevelChange()
            {
                return new IntVariable(proposedLevelChange);
            }
        }
        public Attribute Attribute;
        public IntVariable Level;
        public IntVariable MaxLevel;
        public IntVariable ProposedLevelChange;
        public abstract string StatName { get; }

        [SerializeField] private Attributes allAttributes;

        private void OnEnable()
        {
            Load();
        }

        private void OnDisable()
        {
            Save();
        }

        public virtual int GetCost(int targetLevel)
        {
            int currentLevel = Level.Value;
            int cost = 0;
            int start;
            int bound;
            bool isRefund = false;

            if (targetLevel < currentLevel)
            {
                start = targetLevel + 1;
                bound = currentLevel;
                isRefund = true;
            }
            else if (targetLevel > currentLevel)
            {
                start = currentLevel + 1;
                bound = targetLevel;
            }
            else
            {
                return 0;
            }

            for (int i = start; i <= bound; i++)
            {
                cost += i * 10;
            }

            if (isRefund)
            {
                cost *= -1;
            }
            return cost;
        }

        public void Refresh()
        {
            MaxLevel = Attribute.Level;
        }

        public void Save(bool saveDefault = false)
        {
            JSONUtil.SaveData(new DataContainer(Attribute, Level, MaxLevel, ProposedLevelChange), $"{(saveDefault ? "Default/" : "")}Attribute/Stat/{StatName}.json");
        }

        public void Load()
        {
            DataContainer dataContainer = JSONUtil.LoadData<DataContainer>($"Attribute/Stat/{StatName}.json");
            Attribute = dataContainer.GetAttribute(allAttributes);
            Level = dataContainer.GetLevel();
            MaxLevel = dataContainer.GetMaxLevel();
            ProposedLevelChange = dataContainer.GetProposedLevelChange();
        }
    } 
}

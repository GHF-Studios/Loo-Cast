using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Attribute.Stat
{
    using LooCast.Variable;

    [Serializable]
    public abstract class Stat : ScriptableObject
    {
        public class DataContainer
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

        public abstract string StatName { get; }

        public Attribute Attribute;
        public IntVariable Level;
        public IntVariable MaxLevel;
        public IntVariable ProposedLevelChange;

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
    } 
}

using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Attribute
{
    using LooCast.Attribute.Stat;
    using LooCast.Variable;

    [Serializable]
    public abstract class Attribute : ScriptableObject
    {
        public class DataContainer
        {
            [SerializeField] private string[] statNames;
            [SerializeField] private int level;
            [SerializeField] private int maxLevel;
            [SerializeField] private int proposedLevelChange;

            public DataContainer(Stat.Stat[] stats, IntVariable level, IntVariable maxLevel, IntVariable proposedLevelChange)
            {
                statNames = new string[stats.Length];
                for (int i = 0; i < stats.Length; i++)
                {
                    statNames[i] = stats[i].StatName;
                }
                this.level = level.Value;
                this.maxLevel = maxLevel.Value;
                this.proposedLevelChange = proposedLevelChange.Value;
            }

            public Stat.Stat[] GetStats(Stats allStats)
            {
                Stat.Stat[] stats = new Stat.Stat[statNames.Length];
                for (int i = 0; i < statNames.Length; i++)
                {
                    stats[i] = allStats.GetStat(statNames[i]);
                }
                return stats;
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

        public abstract string AttributeName { get; }

        public Stat.Stat[] Stats;
        public IntVariable Level;
        public IntVariable MaxLevel;
        public IntVariable ProposedLevelChange;

        public int GetCost(int targetLevel)
        {
            int currentLevel = Level.Value;
            int cost = 0;
            int start;
            int end;
            bool isRefund = false;

            if (targetLevel < currentLevel)
            {
                start = targetLevel + 1;
                end = currentLevel;
                isRefund = true;
            }
            else if (targetLevel > currentLevel)
            {
                start = currentLevel + 1;
                end = targetLevel;
            }
            else
            {
                return 0;
            }

            for (int i = start; i <= end; i++)
            {
                cost += i;
            }

            if (isRefund)
            {
                cost *= -1;
            }
            return cost;
        }

        public void Refresh()
        {
            foreach (Stat.Stat Stat in Stats)
            {
                Stat.Refresh();
            }
        }
    }
}

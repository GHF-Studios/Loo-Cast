using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Attribute
{
    using LooCast.Attribute.Stat;
    using LooCast.Variable;
    using LooCast.Data;
    using LooCast.Util;

    [Serializable]
    public abstract class Attribute : ScriptableObject, ISaveable
    {
        private class DataContainer
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
        public Stat.Stat[] Stats;
        public IntVariable Level;
        public IntVariable MaxLevel;
        public IntVariable ProposedLevelChange;
        public abstract string AttributeName { get; }

        [SerializeField] private Stats allStats;

        private void OnEnable()
        {
            Load();
        }

        private void OnDisable()
        {
            Save();
        }

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

        public void Save(bool saveDefault = false)
        {
            JSONUtil.SaveData(new DataContainer(Stats, Level, MaxLevel, ProposedLevelChange), $"{(saveDefault ? "Default/" : "")}Attribute/{AttributeName}.json");
        }

        public void Load()
        {
            DataContainer dataContainer = JSONUtil.LoadData<DataContainer>($"Attribute/{AttributeName}.json");
            Stats = dataContainer.GetStats(allStats);
            Level = dataContainer.GetLevel();
            MaxLevel = dataContainer.GetMaxLevel();
            ProposedLevelChange = dataContainer.GetProposedLevelChange();
        }
    }
}

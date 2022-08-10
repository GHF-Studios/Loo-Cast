using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Attribute
{
    using LooCast.Variable;

    [Serializable]
    public abstract class Attribute : ScriptableObject
    {
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

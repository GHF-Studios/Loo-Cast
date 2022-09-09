using System;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Mission.Task
{
    using LooCast.Mission.Trigger;
    using LooCast.Health;
    
    public class KillXMissionTask : MissionTask
    {
        public int KillCount
        {
            get
            {
                return killCount;
            }

            private set
            {
                killCount = value;
                if (killCount >= requiredKillCount)
                {
                    Complete();
                }
            }
        }
        private MissionTaskState missionTaskState;
        private UnityAction<Type> onKillCountedCall;
        private Type enemyType;
        private int requiredKillCount;
        private int killCount;

        public KillXMissionTask(Type requiredEnemyType, int requiredKillCount) : base($"Kill {requiredKillCount} {requiredEnemyType.Name}")
        {
            missionTaskState = MissionTaskState.Incomplete;
            this.enemyType = requiredEnemyType;
            this.requiredKillCount = requiredKillCount;
            killCount = 0;
            onKillCountedCall = (enemyType) =>
            {
                if (enemyType == requiredEnemyType)
                {
                    KillCount++;
                }
            };
            EnemyHealth.OnKillCounted.AddListener(onKillCountedCall);
        }

        public override void Complete()
        {
            base.Complete();
            EnemyHealth.OnKillCounted.RemoveListener(onKillCountedCall);
        }
    }
}
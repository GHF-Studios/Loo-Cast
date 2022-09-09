using System;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Mission.Task
{
    using LooCast.Mission.Trigger;
    using LooCast.Health;
    using LooCast.Enemy;
    
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
        private Type requiredEnemyType;
        private int requiredKillCount;
        private int killCount;

        public KillXMissionTask(Type requiredEnemyType, int requiredKillCount) : base($"Kill {requiredKillCount} {requiredEnemyType.Name}")
        {
            if (!IsValidType(requiredEnemyType, typeof(Enemy)))
            {
                throw new ArgumentException("Required Enemy Type provided is not Enemy, or a Subclass of Enemy!");
            }
            missionTaskState = MissionTaskState.Incomplete;
            this.requiredEnemyType = requiredEnemyType;
            this.requiredKillCount = requiredKillCount;
            killCount = 0;
            onKillCountedCall = (enemyType) =>
            {
                if (IsValidType(enemyType, requiredEnemyType) && MissionTaskState != MissionTaskState.Locked)
                {
                    KillCount++;
                }
            };
            Enemy.OnKillCounted.AddListener(onKillCountedCall);
        }

        public override void Complete()
        {
            base.Complete();
            Enemy.OnKillCounted.RemoveListener(onKillCountedCall);
        }

        private bool IsValidType(Type type, Type validationType)
        {
            if (!type.IsSubclassOf(validationType) && type != validationType)
            {
                return false;
            }
            return true;
        }
    }
}
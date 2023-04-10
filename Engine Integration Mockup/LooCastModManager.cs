using System;
using UnityEngine;

namespace LooCast.Core
{
    [ModManager]
    public class LooCastModManager : ModManager
    {
        #region Manager Types

        [ManagerTypeInstance]
        public static ManagerType HealthManagerType;

        [ManagerTypeInstance]
        public static ManagerType MovementManagerType;

        [ManagerTypeInstance]
        public static ManagerType ParticleSystemManagerType;

        [ManagerTypeInstance]
        public static ManagerType VFXManagerType;
        
        [ManagerTypeInstance]
        public static ManagerType EnemyManagerType;

        [ManagerTypeInstance]
        public static ManagerType AllyManagerType;

        [ManagerTypeInstance]
        public static ManagerType PlayerManagerType;
        #endregion
    } 
}

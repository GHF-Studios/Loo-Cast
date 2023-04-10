using System;
using UnityEngine;
using LooCast.Core;
using LooCast.Particle;

namespace LooCast.VFX
{
    [ModuleManager]
    public class VFXModuleManager : ModuleManager
    {
        #region Types

        #region MetaData Types
        #endregion

        #region Persistable Entity Types
        #endregion

        #region Non-Persistable Entity Types
        [EntityTypeInstance]
        public static ExplosionEntityType ExplosionEntityType;
        #endregion

        #region Persistable Data Types
        #endregion

        #region Non-Persistable Data Types
        [DataTypeInstance]
        public static ExplosionDataType ExplosionDataType;

        [DataTypeInstance]
        public static ExplosionParticleSystemDataType ExplosionParticleSystemDataType;
        #endregion

        #region Logic Types
        [LogicTypeInstance]
        public static ExplosionLogicType ExplosionLogicType;

        [LogicTypeInstance]
        public static ExplosionParticleSystemLogicType ExplosionParticleSystemLogicType;
        #endregion

        #endregion
    } 
}

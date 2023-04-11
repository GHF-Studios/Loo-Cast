using System;
using UnityEngine;
using LooCast.Core;

namespace LooCast.Particle
{
    [ModuleManager]
    public class ParticleModuleManager : ModuleManager
    {
        #region Sub Module Managers

        #endregion

        #region Entity Types
        #endregion

        #region Component Types
        [ComponentTypeInstance]
        public static ParticleSystemComponentType<ParticleSystemComponent> ParticleSystemComponentType;
        #endregion

        #region MetaData Types
        #endregion

        #region Data Types
        [DataTypeInstance]
        public static ParticleSystemDataType<ParticleSystemData> ParticleSystemDataType;
        #endregion

        #region Logic Types
        [LogicTypeInstance]
        public static ParticleSystemLogicType<ParticleSystemLogic> ParticleSystemLogicType;
        #endregion
    } 
}

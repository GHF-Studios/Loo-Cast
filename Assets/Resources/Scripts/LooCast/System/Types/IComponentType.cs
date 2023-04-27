using System;

namespace LooCast.System.Types
{
    using LooCast.System.Data;
    using LooCast.System.MetaData;
    
    public interface IComponentType : IInstanceType
    {
        #region Interfaces
        public interface IComponent : IInstanceType.IInstance
        {
            #region Properties
            public ComponentMetaData ComponentMetaData { get; set; }

            public ComponentData ComponentData { get; set; }
            #endregion
        }
        #endregion

        #region Properties
        public ComponentTypeMetaData ComponentTypeMetaData { get; set; }

        public ComponentTypeData ComponentTypeData { get; set; }
        #endregion
    }
}

using System;

namespace LooCast.Core.Types
{
    using LooCast.Core.Data;
    using LooCast.Core.MetaData;
    
    public interface IComponentType : IInstanceType
    {
        #region Interfaces
        public interface IComponent : IInstanceType.IInstance
        {
            #region Properties
            public IComponentMetaData ComponentMetaData { get; set; }

            public IComponentData ComponentData { get; set; }
            #endregion
        }
        #endregion

        #region Properties
        public IComponentTypeMetaData ComponentTypeMetaData { get; set; }

        public IComponentTypeData ComponentTypeData { get; set; }
        #endregion
    }
}

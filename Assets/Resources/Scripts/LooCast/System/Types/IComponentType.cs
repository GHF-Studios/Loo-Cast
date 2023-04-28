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

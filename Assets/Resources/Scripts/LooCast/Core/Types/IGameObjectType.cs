using System;

namespace LooCast.Core.Types
{
    using LooCast.Core.Data;
    using LooCast.Core.MetaData;

    public interface IGameObjectType : IInstanceType
    {
        #region Interfaces
        public interface IGameObject : IInstanceType.IInstance
        {
            #region Properties
            public IGameObjectMetaData GameObjectMetaData { get; set; }

            public IGameObjectData GameObjectData { get; set; }
            #endregion
        }
        #endregion

        #region Properties
        public IGameObjectTypeMetaData GameObjectTypeMetaData { get; set; }

        public IGameObjectTypeData GameObjectTypeData { get; set; }
        #endregion
    }
}

using System;

namespace LooCast.System.Types
{
    using LooCast.System.Data;
    using LooCast.System.MetaData;

    public interface IGameObjectType : IInstanceType
    {
        #region Interfaces
        public interface IGameObject : IInstanceType.IInstance
        {
            #region Properties
            public GameObjectMetaData GameObjectMetaData { get; set; }

            public GameObjectData GameObjectData { get; set; }
            #endregion
        }
        #endregion

        #region Properties
        public GameObjectTypeMetaData GameObjectTypeMetaData { get; set; }

        public GameObjectTypeData GameObjectTypeData { get; set; }
        #endregion
    }
}

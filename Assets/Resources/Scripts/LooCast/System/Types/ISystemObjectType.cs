using System;

namespace LooCast.System.Types
{
    using LooCast.System.Data;
    using LooCast.System.MetaData;

    public interface ISystemObjectType : IInstanceType
    {
        #region Interfaces
        public interface ISystemObject : IInstanceType.IInstance
        {
            #region Properties
            public ISystemObjectMetaData SystemObjectMetaData { get; set; }

            public ISystemObjectData SystemObjectData { get; set; }
            #endregion
        }
        #endregion

        #region Properties
        public ISystemObjectTypeMetaData SystemObjectTypeMetaData { get; set; }

        public ISystemObjectTypeData SystemObjectTypeData { get; set; }
        #endregion
    }
}

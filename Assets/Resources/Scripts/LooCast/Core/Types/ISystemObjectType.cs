using System;

namespace LooCast.Core.Types
{
    using LooCast.Core.Data;
    using LooCast.Core.MetaData;

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

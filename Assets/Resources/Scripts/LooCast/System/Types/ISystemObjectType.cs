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
            public SystemObjectMetaData SystemObjectMetaData { get; set; }

            public SystemObjectData SystemObjectData { get; set; }
            #endregion
        }
        #endregion

        #region Properties
        public SystemObjectTypeMetaData SystemObjectTypeMetaData { get; set; }

        public SystemObjectTypeData SystemObjectTypeData { get; set; }
        #endregion
    }
}

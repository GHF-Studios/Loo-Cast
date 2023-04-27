using LooCast.System.Data;
using LooCast.System.MetaData;
using System;

namespace LooCast.System.Types
{
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

using System;

namespace LooCast.System.Identification
{
    public interface IObjectMetaDataIdentifiable : ICSharpInstanceMetaDataIdentifiable
    {
        #region Properties
        IObjectMetaDataIdentifiable ObjectMetaDataIdentifier { get; }
        #endregion
    }
}

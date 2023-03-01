using System;

namespace LooCast.System.Identification
{
    public interface IMetaDataIdentifiable : IObjectIdentifiable
    {
        #region Properties
        IMetaDataIdentifier MetaDataIdentifier { get; }
        #endregion
    }
}

using System;

namespace LooCast.System.Identification
{
    public interface IResourceFileDataIdentifiable : IResourceObjectDataIdentifiable
    {
        #region Properties
        IResourceFileDataIdentifier ResourceFileDataIdentifier { get; }
        #endregion
    }
}

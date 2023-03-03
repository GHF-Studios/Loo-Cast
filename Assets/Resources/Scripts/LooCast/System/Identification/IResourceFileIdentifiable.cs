using System;

namespace LooCast.System.Identification
{
    public interface IResourceFileIdentifiable : IResourceObjectIdentifiable
    {
        #region Properties
        IResourceFileIdentifier ResourceFileIdentifier { get; }
        #endregion
    }
}

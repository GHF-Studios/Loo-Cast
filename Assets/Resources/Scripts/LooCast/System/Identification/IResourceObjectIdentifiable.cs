using System;

namespace LooCast.System.Identification
{
    public interface IResourceObjectIdentifiable : IResourceIdentifiable
    {
        #region Properties
        IResourceObjectIdentifier ResourceObjectIdentifier { get; }
        #endregion
    }
}

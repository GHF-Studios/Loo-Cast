using System;

namespace LooCast.System.Identification
{
    public interface IResourceIdentifiable : IObjectIdentifiable
    {
        #region Properties
        IResourceIdentifier ResourceIdentifier { get; }
        #endregion
    }
}

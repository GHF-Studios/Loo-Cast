using System;

namespace LooCast.System.Identification
{
    public interface IResourceDataIdentifiable : IObjectDataIdentifiable
    {
        #region Properties
        IResourceDataIdentifier ResourceDataIdentifier { get; }
        #endregion
    }
}

using System;

namespace LooCast.System.Identification
{
    public interface IResourceObjectDataIdentifiable : IObjectDataIdentifiable
    {
        #region Properties
        IResourceObjectDataIdentifier ResourceObjectDataIdentifier { get; }
        #endregion
    }
}

using System;

namespace LooCast.System.Identification
{
    public interface ITypeIdentifiable : IIdentifiable
    {
        #region Properties
        ITypeIdentifier TypeIdentifier { get; }
        #endregion
    }
}

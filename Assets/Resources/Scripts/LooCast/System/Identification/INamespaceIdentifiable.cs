using System;

namespace LooCast.System.Identification
{
    public interface INamespaceIdentifiable : IIdentifiable
    {
        #region Properties
        INamespaceIdentifier NamespaceIdentifier { get; }
        #endregion
    }
}

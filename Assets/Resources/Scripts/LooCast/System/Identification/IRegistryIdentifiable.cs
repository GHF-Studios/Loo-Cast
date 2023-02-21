using System;

namespace LooCast.System.Identification
{
    public interface IRegistryIdentifiable : IIdentifiable
    {
        #region Properties
        IRegistryIdentifier RegistryIdentifier { get; }
        #endregion
    }
}

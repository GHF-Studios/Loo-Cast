using System;

namespace LooCast.System.Identifiers
{
    public interface IRegistryIdentifier : IIdentifier
    {
        ITypeIdentifier KeyTypeIdentifier { get; }
        ITypeIdentifier ValueTypeIdentifier { get; }
    }
}

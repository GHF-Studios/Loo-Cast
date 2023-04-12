using LooCast.System.Identifiers;
using System;

namespace LooCast.System.Registries
{
    using global::LooCast.System.MetaData;
    using global::LooCast.System.Managers;

    public class MainRegistry : Registry<Identifier, ILooCastObject>
    {
        public MainRegistry() : base(new RegistryMetaData("LooCast.System.Registries"))
        {
        }
    }
}

using System;

namespace LooCast.System.Registries
{
    using LooCast.System.Identifiers;
    
    public class MainRegistry : Registry<IRegistryIdentifier, IRegistry>
    {
        #region Properties
        public MetaDataRegistry MetaDataRegistry { get; private set; }
        public DataRegistry DataRegistry { get; private set; }
        public NamespaceRegistry NamespaceRegistry { get; private set; }
        public TypeRegistry TypeRegistry { get; private set; }
        #endregion

        #region Overrides
        public override void PostInitialize()
        {
            base.PostInitialize();
            
            MetaDataRegistry = Get(Identifiers.RegistryIdentifier.Parse(typeof(IMetaDataIdentifier), typeof(IMetaData))) as MetaDataRegistry;
            DataRegistry = Get(Identifiers.RegistryIdentifier.Parse(typeof(IDataIdentifier), typeof(IData))) as DataRegistry;
            NamespaceRegistry = Get(Identifiers.RegistryIdentifier.Parse(typeof(INamespaceIdentifier), typeof(INamespace))) as NamespaceRegistry;
            TypeRegistry = Get(Identifiers.RegistryIdentifier.Parse(typeof(ITypeIdentifier), typeof(IType))) as TypeRegistry;
        }
        #endregion
    }
}

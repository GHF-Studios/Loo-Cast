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
        
        #region Constructors
        public MainRegistry() : base(null)
        {

        }
        #endregion

        #region Overrides
        public override void PostInitialize()
        {
            base.PostInitialize();

            MetaDataRegistry = GetObject((IObjectIdentifier)Identifiers.RegistryIdentifier.Parse(typeof(IMetaDataIdentifier), typeof(IMetaData))) as MetaDataRegistry;
            DataRegistry = GetObject((IObjectIdentifier)Identifiers.RegistryIdentifier.Parse(typeof(IDataIdentifier), typeof(IData))) as DataRegistry;
            NamespaceRegistry = GetObject((IObjectIdentifier)Identifiers.RegistryIdentifier.Parse(typeof(INamespaceIdentifier), typeof(INamespace))) as NamespaceRegistry;
            TypeRegistry = GetObject((IObjectIdentifier)Identifiers.RegistryIdentifier.Parse(typeof(ITypeIdentifier), typeof(IType))) as TypeRegistry;
        }
        #endregion
    }
}

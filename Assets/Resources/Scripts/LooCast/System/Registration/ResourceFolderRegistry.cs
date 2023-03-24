using System;

namespace LooCast.System.Registration
{
    using LooCast.System.Identification;
    using LooCast.System.Types;

    public class ResourceFolderRegistry : Registry<IResourceFolderIdentifier, IResourceFolderIdentifiable>, IResourceFolderRegistry<IResourceFolderIdentifier, IResourceFolderIdentifiable>
    {
        public ResourceFolderRegistry(IType keyType, IType valueType) : base(keyType, valueType)
        {
            
        }
    }
}

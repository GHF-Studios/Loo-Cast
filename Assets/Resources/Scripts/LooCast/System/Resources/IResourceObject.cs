using System.Collections.Generic;

namespace LooCast.System.Resources
{
    using LooCast.System.Identification;

    public interface IResourceObject : IResource, IResourceObjectIdentifiable
    {
        #region Properties
        public string ResourceObjectPath { get; }
        public IResourceObjectType ResourceObjectType { get; }
        public IResourceObject? ParentResourceObject { get; }
        public IResourceFile? ParentResourceFile { get; }
        public SerializableList<IResourceObject> ChildResourceObjects { get; }
        #endregion
    }
}

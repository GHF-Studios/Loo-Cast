using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.Identification;

    public interface IResourceObject : IResource, IResourceObjectIdentifiable
    {
        #region Properties
        public IResourceObjectType ResourceObjectType { get; }
        public IResourceObject? ParentResourceObject { get; }
        public IResourceFile? ParentResourceFile { get; }
        public SerializableList<IResourceObject> ChildResourceObjects { get; }
        public string ResourceObjectPath { get; }
        #endregion
    }
}

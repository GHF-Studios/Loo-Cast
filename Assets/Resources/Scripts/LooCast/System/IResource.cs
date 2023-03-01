using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.Identification;

    public interface IResource : IObject, IResourceIdentifiable
    {
        #region Properties
        public IResourceType ResourceType { get; }
        public IResource ParentResource { get; }
        public SerializableList<IResource> ChildResources { get; }
        public string ResourcePath { get; }
        #endregion

        #region Methods
        public string SerializeRecursively();
        public void DeserializeRecursively(string serializedResource);
        #endregion
    }
}

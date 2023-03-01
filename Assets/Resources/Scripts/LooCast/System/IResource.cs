using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.Identification;

    public interface IResource : IObject, IResourceType, IResourceIdentifiable
    {
        #region Properties
        public string ResourcePath { get; }
        public IResource? ParentResource { get; }
        public SerializableList<IResource> ChildResources { get; }
        #endregion

        #region Methods
        // TODO:    These two methods are likely only abstractions at best, and probably need to 
        //          parse more information, like what Serializer is used, what Type it serializes into, etc.
        public string SerializeRecursively();
        public void DeserializeRecursively(string serializedResource);
        #endregion
    }
}

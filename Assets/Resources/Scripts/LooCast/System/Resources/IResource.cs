using System.Collections.Generic;

namespace LooCast.System.Resources
{
    using LooCast.System.Collections.Generic;
    using LooCast.System.Identification;

    public interface IResource : IObject, IResourceIdentifiable
    {
        #region Properties
        public string ResourcePath { get; }
        public IResource? ParentResource { get; }
        public SerializableList<IResource> ChildResources { get; }
        public ResourceKind ResourceKind { get; }
        public ResourceLoadState ResourceLoadState { get; }
        #endregion

        #region Methods
        public void Load();
        public void Unload();
        public void Save();
        #endregion
    }
}

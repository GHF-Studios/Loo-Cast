using System.Collections.Generic;

namespace LooCast.System.Resources
{
    using LooCast.System.Identification;

    public interface IResource : IObject, IResourceIdentifiable
    {
        #region Properties
        public string ResourcePath { get; }
        public IResource? ParentResource { get; }
        public SerializableList<IResource> ChildResources { get; }
        #endregion

        #region Methods
        public void Load();
        public void Save();
        #endregion
    }
}

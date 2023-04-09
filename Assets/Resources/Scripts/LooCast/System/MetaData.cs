using System;
using System.Collections.Generic;

namespace LooCast.System
{
    using global::LooCast.System.Managers;

    [Serializable]
    public abstract class MetaData : IIdentifiable
    {
        #region Fields
#nullable enable
        public List<Identifier>? DependencyIdentifiers { get; }
        public List<System.MetaData>? Dependencies { get; }
#nullable disable
        #endregion

        #region Methods
        public virtual void Register()
        {
            MainManager mainManager = MainManager.Instance;
            mainManager.MainRegistry.Add(Identifier, this);
            // penis
            // get metadata from the location that is specified by the identifer (as it is a unique identifier, it can also uniquely identify the location) and then register the metadata.
            // Effectively this means that metadata is registered in a recursive way, so this method will be called on all dependencies of this metadata, which will call this method on all dependencies of those dependencies, and so on.
            // When the metadata has been registered, it will be added to the Dependencies list of this metadata.

            // But before doing so, I am required to create HierarchyElementPath, HierarchyFolderPath, HierarchyFilePath, HierarchyObjectPath, and HierarchyElement
            // basically acting as another identifier system but for paths in a hierarchy
            // Also make this interface be an abstract class, somehow
        }

        public virtual void Validate()
        {
            if (DependencyIdentifiers == null || DependencyIdentifiers.Count == 0)
            {
                return;
            }
            
            foreach (Identifier dependencyIdentifier in DependencyIdentifiers)
            {
                ValidateDependency(dependencyIdentifier);
            }
        }

        public virtual void ValidateDependency(Identifier dependencyIdentifier)
        {
            if (dependencyIdentifier == null)
            {
                throw new ArgumentNullException(nameof(dependencyIdentifier));
            }

            MainManager mainManager = MainManager.Instance;
            if (!mainManager.MainRegistry.ContainsKey(dependencyIdentifier))
            {
                throw new ArgumentException($"Dependency {dependencyIdentifier} could not be found!");
            }
        }
        #endregion
    }
}

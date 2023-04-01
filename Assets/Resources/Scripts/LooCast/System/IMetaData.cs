﻿using System;
using System.Collections.Generic;

namespace LooCast.System
{
    using global::LooCast.System.Identifiers;
    using global::LooCast.System.Managers;
    
    public interface IMetaData : IIdentifiable
    {
        #region Fields
#nullable enable
        public List<Identifier>? DependencyIdentifiers { get; set; }
        public List<IMetaData>? Dependencies { get; protected set; }
#nullable disable
        #endregion

        #region Methods
        public virtual void Register()
        {
            MainManager mainManager = MainManager.Instance;
            mainManager.MainRegistry.Add(Identifier, this);
            penis
            // get metadata from the location that is specified by the identifer (as it is a unique identifier, it can also uniquely identify the location) and then register the metadata.
            // Effectively this means that metadata is registered in a recursive way, so this method will be called on all dependencies of this metadata, which will call this method on all dependencies of those dependencies, and so on.
            // When the metadata has been registered, it will be added to the Dependencies list of this metadata.
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

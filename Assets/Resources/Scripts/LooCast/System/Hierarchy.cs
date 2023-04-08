﻿using System.Collections.Generic;

namespace LooCast.System
{
    public abstract class Hierarchy<ElementType> : SystemObject, IHierarchyElement where ElementType : class, IHierarchyElement
    {
        #region Properties
        #endregion

        #region Constructors
        #endregion

        #region Public Methods
        public bool TryRegisterElement(ElementType element)
        {
            
        }

        public bool TryUnregisterElement(HierarchyElementPath hierarchyElementPath)
        {
            
        }

        public bool TryGetElement(HierarchyElementPath hierarchyElementPath, out ElementType? element)
        {
            
        }
        #endregion
    }
}

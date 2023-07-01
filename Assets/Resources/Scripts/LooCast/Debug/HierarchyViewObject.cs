﻿using UnityEngine;
using LooCast.System;
using System.Collections.Generic;

public class HierarchyViewObject : HierarchyViewElement
{
    #region Fields
    [SerializeField] private GameObject hierarchyViewObjectPrefab;

    private IObject hierarchyObject;
    private Dictionary<string, HierarchyViewObject> hierarchyViewObjectChildren;
    #endregion

    #region Methods
    public void Initialize(IObject hierarchyObject)
    {
        base.Initialize(hierarchyObject.ObjectName);
        
        this.hierarchyObject = hierarchyObject;
        hierarchyViewObjectChildren = new Dictionary<string, HierarchyViewObject>();

        InstantiateChildren();
    }
    #endregion

    #region Overrides
    protected override void InstantiateChildren()
    {
        foreach (IObject _object in hierarchyObject.Children)
        {
            HierarchyViewObject hierarchyViewObject = Instantiate(hierarchyViewObjectPrefab, elementContainer.transform).GetComponent<HierarchyViewObject>();
            hierarchyViewObject.gameObject.name = _object.ObjectName;
            hierarchyViewObjectChildren.Add(_object.ObjectName, hierarchyViewObject);
        }

        if (hierarchyViewObjectChildren.Count == 0)
        {
            hasAnyChildren = false;
        }
        else
        {
            hasAnyChildren = true;
        }

        foreach (IObject _object in hierarchyObject.Children)
        {
            hierarchyViewObjectChildren.TryGetValue(_object.ObjectName, out HierarchyViewObject hierarchyViewObject);
            hierarchyViewObject.Initialize(_object);
        }
    }
    #endregion
}

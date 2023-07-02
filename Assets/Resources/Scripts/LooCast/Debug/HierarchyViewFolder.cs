using UnityEngine;
using LooCast.System;
using System.Collections.Generic;
using UnityEngine.UI;

public class HierarchyViewFolder : HierarchyViewElement
{
    #region Fields
    [SerializeField] private GameObject hierarchyViewFolderPrefab;
    [SerializeField] private GameObject hierarchyViewFilePrefab;

    private IFolder hierarchyFolder;
    private Dictionary<string, HierarchyViewFolder> hierarchyViewFolderChildren;
    private Dictionary<string, HierarchyViewFile> hierarchyViewFileChildren;
    #endregion

    #region Methods
    public void Initialize(IFolder hierarchyFolder)
    {
        base.Initialize(hierarchyFolder.FolderName);
        
        this.hierarchyFolder = hierarchyFolder;
        hierarchyViewFolderChildren = new Dictionary<string, HierarchyViewFolder>();
        hierarchyViewFileChildren = new Dictionary<string, HierarchyViewFile>();
    }
    #endregion

    #region Overrides
    protected override void InstantiateChildren()
    {
        base.InstantiateChildren();

        foreach (IFolder folder in ((IParent<IFolder>)hierarchyFolder).Children)
        {
            HierarchyViewFolder hierarchyViewFolder = Instantiate(hierarchyViewFolderPrefab, elementContainer.transform).GetComponent<HierarchyViewFolder>();
            LayoutRebuilder.MarkLayoutForRebuild((RectTransform)transform);
            elementContainerLayoutGroup.CalculateLayoutInputHorizontal();
            elementContainerLayoutGroup.CalculateLayoutInputVertical();

            hierarchyViewFolder.gameObject.name = folder.FolderName;
            hierarchyViewFolderChildren.Add(folder.FolderName, hierarchyViewFolder);
            hierarchyViewFolder.Initialize(folder);
            LayoutRebuilder.MarkLayoutForRebuild((RectTransform)transform);
            elementContainerLayoutGroup.CalculateLayoutInputHorizontal();
            elementContainerLayoutGroup.CalculateLayoutInputVertical();
        }

        foreach (IFile file in ((IParent<IFile>)hierarchyFolder).Children)
        {
            HierarchyViewFile hierarchyViewFile = Instantiate(hierarchyViewFilePrefab, elementContainer.transform).GetComponent<HierarchyViewFile>();
            LayoutRebuilder.MarkLayoutForRebuild((RectTransform)transform);
            elementContainerLayoutGroup.CalculateLayoutInputHorizontal();
            elementContainerLayoutGroup.CalculateLayoutInputVertical();

            hierarchyViewFile.gameObject.name = file.FileIdentifier;
            hierarchyViewFileChildren.Add(file.FileIdentifier, hierarchyViewFile);
            hierarchyViewFile.Initialize(file);
            LayoutRebuilder.MarkLayoutForRebuild((RectTransform)transform);
            elementContainerLayoutGroup.CalculateLayoutInputHorizontal();
            elementContainerLayoutGroup.CalculateLayoutInputVertical();
        }

        if (hierarchyViewFolderChildren.Count != 0 || hierarchyViewFileChildren.Count != 0)
        {
            hasAnyChildren = true;
        }
    }
    #endregion
}

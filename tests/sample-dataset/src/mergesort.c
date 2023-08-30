#include <stdio.h>
#include <stdlib.h>
#define MAX 50

void mergeSort(int arr[], int low, int mid, int high);
void partition(int arr[], int low, int high);

int main(int argc, char *argv[]) {
  if (argc < 11)
    return 1;
  int merge[10], i;

  for (i = 1; i < argc; i++) {
    merge[i - 1] = (int) strtol(argv[i], NULL, 10);
  }

  partition(merge, 0, argc - 2);

  printf("After merge sorting elements are: ");
  for (i = 0; i < argc - 1; i++) {
    printf("%d ", merge[i]);
  }

  return 0;
}

void partition(int arr[], int low, int high) {

  int mid;

  if (low < high) {
    mid = low + (high - low) / 2;
    partition(arr, low, mid);
    partition(arr, mid + 1, high);
    mergeSort(arr, low, mid, high);
  }
}

void mergeSort(int arr[], int low, int mid, int high) {

  int i, m, k, l, temp[MAX];

  l = low;
  i = low;
  m = mid + 1;

  while ((l <= mid) && (m <= high)) {

    if (arr[l] <= arr[m]) {
      temp[i] = arr[l];
      l++;
    } else {
      temp[i] = arr[m];
      m++;
    }
    i++;
  }

  if (l > mid) {
    for (k = m; k <= high; k++) {
      temp[i] = arr[k];
      i++;
    }
  } else {
    for (k = l; k <= mid; k++) {
      temp[i] = arr[k];
      i++;
    }
  }
}

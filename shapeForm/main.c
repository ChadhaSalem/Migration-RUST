#include <stdio.h>
#include<stdint.h>
#include<stdbool.h>
uint16_t shape_form_recognition(float *pDataIn, uint16_t *pPeakLocs,float *pPeakValues, uint16_t startIndex, uint16_t endIndex, uint16_t winMin, uint16_t winMax)
{
        uint16_t temp,temp_saved,temp_start;
        uint16_t numPeaks;
        numPeaks = 0;
        float min1,min2;
        bool t_minima1=0,t_maxima1=0,t_minima2=0,t_maxima2=0;

        for (temp = startIndex+1; temp < endIndex-1; temp++)
        {
            if(pDataIn[temp] > pDataIn[temp-1]  & pDataIn[temp] > pDataIn[temp+1])
            {
                if(t_minima1){
                    if(pDataIn[temp] > 0.05){
                        t_maxima1 = 1;
                        t_minima1 = 0;
                    }
                }
                if(t_minima2){
                    t_maxima2 = 1;
                    t_minima2 = 0;
                }
            }
            else if (pDataIn[temp] < pDataIn[temp-1] & pDataIn[temp] < pDataIn[temp+1])//minima
            {
                if(t_maxima2){
                    if(pDataIn[temp] < min2 & pDataIn[temp] < -0.05 & fabs(min1-pDataIn[temp]) < 0.15 & temp-temp_start < winMax & temp-temp_start > winMin){
                        t_maxima2 = 0;
                        pPeakLocs[numPeaks] = temp_saved;//temp;
                        pPeakValues[numPeaks] = (float) pDataIn[temp_saved];//temp];
                        numPeaks = numPeaks+1;
                        temp -= 1;
                    }
                    else{
                        t_maxima2 = 0;
                        temp = temp_saved-1;
                    }
                }
                else{
                    if(!t_maxima1){
                        if(pDataIn[temp] < -0.05){
                            t_minima1 = 1;
                            min1 = pDataIn[temp];
                            temp_start = temp;
                        }
                        else{
                            t_minima1 = 0;
                        }
                    }
                    else{
                        temp_saved = temp;
                        if(pDataIn[temp] > min1){
                            t_minima2 = 1;
                            t_maxima1 = 0;
                            min2 = pDataIn[temp];
                        }
                        else{
                            t_maxima1 = 0;
                            temp -= 1;
                        }
                    }
                }
            }
        }
        return numPeaks;
}